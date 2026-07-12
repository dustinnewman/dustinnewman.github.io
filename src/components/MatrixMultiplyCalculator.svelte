<script>
  import { tick } from "svelte";

  const FLY_MS =
    typeof matchMedia !== "undefined" &&
    matchMedia("(prefers-reduced-motion: reduce)").matches
      ? 0
      : 650;

  let a = $state([
    [1, 2],
    [3, 4],
  ]);
  let b = $state([
    [5, 6],
    [7, 8],
  ]);
  let aText = $state("");
  let bText = $state("");
  let aError = $state(false);
  let bError = $state(false);
  let hovered = $state(null);
  let root = $state(null);

  // "idle" → full product shown; "running" → stepping; "done" → last step landed
  let phase = $state("idle");
  let step = $state(0);
  let flying = $state(false);
  let landed = $state(false);
  let chips = $state([]);
  let flightId = 0;

  let conformable = $derived(a[0].length === b.length);
  let product = $derived(
    conformable
      ? a.map((row) =>
          b[0].map((_, j) =>
            row.reduce((sum, value, k) => sum + Number(value) * Number(b[k][j]), 0)
          )
        )
      : null
  );
  let total = $derived(a.length * b[0].length);
  // Steps walk column j of B down the rows i of A: step = j * rows + i
  let cur = $derived(
    phase === "idle" ? null : { i: step % a.length, j: Math.floor(step / a.length) }
  );

  function cellStep(i, j) {
    return j * a.length + i;
  }

  function rectOf(selector, rootRect) {
    const r = root.querySelector(selector).getBoundingClientRect();
    return {
      x: r.left - rootRect.left,
      y: r.top - rootRect.top,
      w: r.width,
      h: r.height,
    };
  }

  // The whole of column `col` in B, as one vector-shaped box.
  function columnStrip(col, rootRect) {
    const cells = b.map((_, k) => rectOf(`[data-cell="b-${k}-${col}"]`, rootRect));
    const first = cells[0];
    const last = cells[cells.length - 1];
    return {
      x: first.x,
      y: first.y,
      w: first.w,
      h: last.y + last.h - first.y,
      values: b.map((row) => Number(row[col])),
    };
  }

  function rowBox(i, rootRect) {
    const first = rectOf(`[data-cell="a-${i}-0"]`, rootRect);
    const last = rectOf(`[data-cell="a-${i}-${a[0].length - 1}"]`, rootRect);
    return {
      cx: (first.x + last.x + last.w) / 2,
      cy: first.y + first.h / 2,
      w: last.x + last.w - first.x,
      h: first.h,
    };
  }

  // A flight of column `col`: upright in its slot in B (row === null), or
  // tipped over 90° and stretched to cover row `row` of A exactly. Rotation
  // happens about the box's center, so translate targets place that center;
  // the box's local height becomes its horizontal extent once rotated.
  function stripFlight(kind, col, fromRow, toRow, rootRect) {
    const strip = columnStrip(col, rootRect);
    const place = (row) => {
      if (row === null)
        return { x: strip.x, y: strip.y, r: 0, w: strip.w, h: strip.h };
      const box = rowBox(row, rootRect);
      return {
        x: box.cx - box.h / 2,
        y: box.cy - box.w / 2,
        r: -90,
        w: box.h,
        h: box.w,
      };
    };
    const from = place(fromRow);
    const to = place(toRow);
    return {
      key: `${kind}-${col}`,
      values: strip.values,
      x0: from.x,
      y0: from.y,
      r0: from.r,
      w0: from.w,
      h0: from.h,
      x1: to.x,
      y1: to.y,
      r1: to.r,
      w1: to.w,
      h1: to.h,
    };
  }

  async function beginStep(next, prev) {
    const token = ++flightId;
    flying = true;
    landed = false;
    // Let landed terms revert to inputs before measuring, so the geometry
    // matches what's on screen for the whole flight.
    await tick();
    if (token !== flightId) return;
    const rootRect = root.getBoundingClientRect();
    const m = a.length;
    const i = next % m;
    const j = Math.floor(next / m);
    const sameColumn = prev !== null && Math.floor(prev / m) === j;
    chips = [
      sameColumn
        ? stripFlight("hop", j, prev % m, i, rootRect)
        : stripFlight("out", j, null, i, rootRect),
    ];
    setTimeout(() => {
      if (token === flightId) finishFlight();
    }, FLY_MS + 80);
  }

  function finishFlight() {
    flightId += 1;
    chips = [];
    flying = false;
    landed = true;
    if (step === total - 1) phase = "done";
  }

  function reset() {
    flightId += 1;
    chips = [];
    flying = false;
    landed = false;
    step = 0;
    phase = "idle";
  }

  function advance() {
    if (flying) {
      finishFlight();
    } else if (phase === "idle") {
      phase = "running";
      step = 0;
      beginStep(0, null);
    } else if (phase === "running") {
      const prev = step;
      step += 1;
      beginStep(step, prev);
    } else {
      reset();
    }
  }

  function parseMatrix(text) {
    let value;
    try {
      value = JSON.parse(text);
    } catch {
      return null;
    }
    if (!Array.isArray(value) || value.length === 0) return null;
    if (value.every((x) => typeof x === "number" && Number.isFinite(x))) {
      return [value];
    }
    const width = Array.isArray(value[0]) ? value[0].length : 0;
    if (width === 0) return null;
    const valid = value.every(
      (row) =>
        Array.isArray(row) &&
        row.length === width &&
        row.every((x) => typeof x === "number" && Number.isFinite(x))
    );
    return valid ? value : null;
  }

  function applyText(which, { showError = false } = {}) {
    const text = (which === "a" ? aText : bText).trim();
    if (text === "") return;
    const parsed = parseMatrix(text);
    if (parsed === null) {
      if (which === "a") aError = showError;
      else bError = showError;
      return;
    }
    reset();
    if (which === "a") {
      a = parsed;
      aError = false;
    } else {
      b = parsed;
      bError = false;
    }
  }

  let label = $derived(
    phase === "idle" ? "Compute" : phase === "done" ? "Restart" : "Next"
  );
</script>

<div class="matmul" bind:this={root}>
  <div class="matrix m-a">
    <table>
      <tbody>
        {#each a as row, i}
          <tr>
            {#each row as _, j}
              <td
                data-cell={`a-${i}-${j}`}
                class:trace={phase === "idle" && hovered?.i === i}
              >
                {#if landed && cur?.i === i}
                  <span class="term">
                    {j > 0 ? "+ " : ""}{Number(a[i][j])}·{Number(b[j][cur.j])}
                  </span>
                {:else}
                  <input bind:value={a[i][j]} readonly={phase !== "idle"} />
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <span class="operator">×</span>

  <div class="matrix m-b">
    <table>
      <tbody>
        {#each b as row, i}
          <tr>
            {#each row as _, j}
              <td
                data-cell={`b-${i}-${j}`}
                class:trace={phase === "idle" && hovered?.j === j}
              >
                <input bind:value={b[i][j]} readonly={phase !== "idle"} />
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <span class="operator">=</span>

  {#if conformable}
    <div class="matrix m-ab" aria-live="polite">
      <table>
        <tbody>
          {#each product as row, i}
            <tr>
              {#each row as value, j}
                <td
                  class="result"
                  class:trace={phase === "idle" &&
                    hovered?.i === i &&
                    hovered?.j === j}
                  onmouseenter={() => (hovered = { i, j })}
                  onmouseleave={() => (hovered = null)}
                >
                  {#if phase === "idle" || cellStep(i, j) < step || (cellStep(i, j) === step && landed)}
                    {value}
                  {:else}
                    <span class="pending">{value}</span>
                  {/if}
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <p class="mismatch">
      A has {a[0].length} column{a[0].length === 1 ? "" : "s"} but B has {b.length}
      row{b.length === 1 ? "" : "s"} — the product is undefined.
    </p>
  {/if}

  {#each chips as c (c.key)}
    <div
      class="chip"
      style="--x0:{c.x0}px; --y0:{c.y0}px; --x1:{c.x1}px; --y1:{c.y1}px; --r0:{c.r0}deg; --r1:{c.r1}deg; --w0:{c.w0}px; --h0:{c.h0}px; --w1:{c.w1}px; --h1:{c.h1}px; animation-duration:{FLY_MS}ms;"
    >
      {#each c.values as v, k (k)}
        <span
          class="chip-cell"
          style="--s0:{-c.r0}deg; --s1:{-c.r1}deg; animation-duration:{FLY_MS}ms;"
        >
          {v}
        </span>
      {/each}
    </div>
  {/each}
</div>

<div class="controls">
  <label>
    A =
    <input
      type="text"
      placeholder="[[1,2],[3,4]]"
      bind:value={aText}
      oninput={() => applyText("a")}
      onchange={() => applyText("a", { showError: true })}
    />
  </label>
  <label>
    B =
    <input
      type="text"
      placeholder="[[5,6],[7,8]] or [5,6] for a row vector"
      bind:value={bText}
      oninput={() => applyText("b")}
      onchange={() => applyText("b", { showError: true })}
    />
  </label>
  {#if aError || bError}
    <p class="parse-error">
      Couldn't parse {aError ? "A" : "B"} — use [[1,2],[3,4]], or [1,2,3] for a
      row vector.
    </p>
  {/if}
  <button onclick={advance} disabled={!conformable}>{label}</button>
</div>

<p class="hint">
  Edit cells directly, or type a whole matrix into A or B. You can see the corresponding rows and columns for the output by hovering (or tapping on) over each output cell.
</p>

<style>
  .matmul {
    /* Row of A is red, column of B is blue; where they meet mixes to purple. */
    --red: #e0483e;
    --blue: #4a90d9;
    --purple: color-mix(in srgb, var(--red) 50%, var(--blue));
    position: relative;
    display: flex;
    align-items: center;
    gap: 1.2rem;
    row-gap: 2rem;
    flex-wrap: wrap;
    margin: 2rem 0;
  }
  .matrix {
    position: relative;
    padding: 0.3rem 1rem;
  }
  .matrix::before,
  .matrix::after {
    content: "";
    position: absolute;
    top: 0;
    bottom: 0;
    width: 0.7rem;
    border-top: 2px solid var(--fg, currentColor);
    border-bottom: 2px solid var(--fg, currentColor);
  }
  .matrix::before {
    left: 0;
    border-left: 2px solid var(--fg, currentColor);
  }
  .matrix::after {
    right: 0;
    border-right: 2px solid var(--fg, currentColor);
  }
  .matmul td {
    padding: 0.4rem 0.6rem;
    text-align: center;
    white-space: nowrap;
  }
  .m-a td {
    min-width: 6.2rem;
  }
  .m-a td.trace {
    background-color: color-mix(in srgb, var(--red) 30%, transparent);
  }
  .m-b td.trace {
    background-color: color-mix(in srgb, var(--blue) 30%, transparent);
  }
  .m-ab td.trace {
    background-color: color-mix(in srgb, var(--purple) 40%, transparent);
  }
  .matmul td input {
    width: 5rem;
    text-align: center;
    font: inherit;
    color: inherit;
    background: transparent;
    border: none;
    border-bottom: 1px solid transparent;
    padding: 0.1rem 0;
  }
  .matmul td input:focus {
    outline: none;
    border-bottom: 1px solid var(--fg, currentColor);
  }
  .matmul td input[readonly] {
    border-bottom-color: transparent;
  }
  .matmul td.result {
    min-width: 5rem;
    cursor: default;
  }
  .pending {
    visibility: hidden;
  }
  /* The column of B in flight: one rigid vector, brackets and all. The box
     rotates about its center; each value counter-rotates to stay upright. */
  .chip {
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    flex-direction: column;
    background-color: var(--bg, #fefefe);
    animation-name: strip-fly;
    animation-timing-function: ease-in-out;
    animation-fill-mode: both;
    pointer-events: none;
  }
  .chip::before,
  .chip::after {
    content: "";
    position: absolute;
    top: 0;
    bottom: 0;
    width: 0.5rem;
    border-top: 2px solid var(--fg, currentColor);
    border-bottom: 2px solid var(--fg, currentColor);
  }
  .chip::before {
    left: 0;
    border-left: 2px solid var(--fg, currentColor);
  }
  .chip::after {
    right: 0;
    border-right: 2px solid var(--fg, currentColor);
  }
  .chip-cell {
    flex: 1 1 0;
    display: flex;
    align-items: center;
    justify-content: center;
    animation-name: strip-spin;
    animation-timing-function: ease-in-out;
    animation-fill-mode: both;
  }
  @keyframes strip-fly {
    from {
      width: var(--w0);
      height: var(--h0);
      transform: translate(var(--x0), var(--y0)) rotate(var(--r0));
    }
    to {
      width: var(--w1);
      height: var(--h1);
      transform: translate(var(--x1), var(--y1)) rotate(var(--r1));
    }
  }
  @keyframes strip-spin {
    from {
      transform: rotate(var(--s0));
    }
    to {
      transform: rotate(var(--s1));
    }
  }
  .operator {
    font-size: 2.4rem;
  }
  .mismatch {
    font-style: italic;
    font-size: 1.6rem;
  }
  .controls {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin: 1.6rem 0;
    max-width: 44rem;
  }
  .controls label {
    display: flex;
    align-items: baseline;
    gap: 0.8rem;
  }
  .controls label input {
    flex: 1;
    margin-bottom: 0;
  }
  .controls label input:focus {
    margin-bottom: -1px;
  }
  .controls button {
    margin-top: 0.8rem;
    align-self: flex-start;
    min-width: 12rem;
    font-family: inherit;
    font-size: 1.6rem;
  }
  .parse-error {
    font-style: italic;
    font-size: 1.4rem;
    margin: 0;
  }
  .hint {
    font-size: 1.4rem;
  }
</style>
