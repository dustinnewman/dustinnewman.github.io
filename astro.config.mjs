import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import svelte from '@astrojs/svelte';
import sitemap from '@astrojs/sitemap';
import remarkMath from 'remark-math';
import rehypeMathjax from 'rehype-mathjax/svg';
import postAssets from './src/integrations/post-assets.mjs';

export default defineConfig({
  site: 'https://dustinnewman.net',
  integrations: [mdx(), svelte(), sitemap(), postAssets()],
  markdown: {
    // The Hugo site shipped unhighlighted code blocks (highlight = false);
    // codelines.js adds line numbers client-side instead.
    syntaxHighlight: false,
    remarkPlugins: [remarkMath],
    rehypePlugins: [rehypeMathjax],
  },
});
