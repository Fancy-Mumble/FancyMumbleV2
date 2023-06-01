export function createEmbeddedIFrame(url: string): Node {
    const iframe = document.createElement('iframe');
    iframe.src = url;
    iframe.style.minWidth = '480px';
    iframe.style.minHeight = '270px';
    iframe.style.border = 'none';
    iframe.className = 'embedded-iframe';
    iframe.allowFullscreen = true;
    return iframe;
}