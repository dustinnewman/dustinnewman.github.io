document.addEventListener("DOMContentLoaded", function() {
    document.querySelectorAll('pre code').forEach(function(codeBlock) {
        var lineCount = codeBlock.textContent.split('\n').length - 1;
        if (lineCount <= 1) {
            return;
        }
        var lineNumberSpan = document.createElement('span');
        lineNumberSpan.className = 'line-number';
        for (var i = 1; i <= lineCount; i++) {
            var span = document.createElement('span');
            span.textContent = i;
            lineNumberSpan.appendChild(span);
        }
        codeBlock.parentNode.insertBefore(lineNumberSpan, codeBlock);
    });
});
