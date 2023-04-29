import DOMPurify from "dompurify";

class MessageParser {
    private document: Document;
    private elements: Array<JSX.Element> = [];

    constructor(input: string) {
        let cleanMessage = DOMPurify.sanitize(input);
        const parser = new DOMParser();
        this.document = parser.parseFromString(cleanMessage, "text/html");
    }

    parseForImages() {
        const images = Array.from(this.document.querySelectorAll('img')).map(img => img.src);

        this.elements.push(<div>
            {images.map(e =>
                <img key={e} src={e} style={{ maxWidth: '100%' }} />
            )}
        </div>);

        return this;
    }

    parseForLinks() {
        const links = Array.from(this.document.querySelectorAll('a')).map(a => a.href);
        // TODO: render preview of links in backend if privacy options allow it

        this.elements.push(<div>
            {links.map(e =>
                <a href={e} target="_blank" rel="noreferrer" style={{ overflowWrap: 'break-word' }}>{e}</a>
            )}
        </div>);

        return this;
    }

    parseForEmojis() {
        /*const emojis = Array.from(this.document.querySelectorAll('img')).map(img => img.src);

        this.elements.push(<div>
            {emojis.map(e =>
                <img key={e} src={e} style={{ maxWidth: '100%' }} />
            )}
        </div>);*/

        return this;
    }

    public build() {
        return this.elements;
    }
}

export default MessageParser;