import { Box } from "@mui/material";
import DOMPurify from "dompurify";
import { marked } from "marked";
import { markedHighlight } from "marked-highlight";


class DOMMessageParser {
    private document: Document;

    constructor(input: string) {
        const parser = new DOMParser();
        this.document = parser.parseFromString(input, "text/html");
    }

    parseForImages() {
        Array.from(this.document.querySelectorAll('img')).forEach(e => {
            e.setAttribute('style', 'max-width: 100%;');
        });

        return this;
    }

    parseForLinks() {
        Array.from(this.document.querySelectorAll('a')).forEach(e => {
            e.setAttribute('target', '_blank');
        });

        return this;
    }

    build() {
        return this.document.documentElement.innerHTML;
    }
}

class IncomingMessageParser {
    private input: string;

    constructor(input: string) {
        this.input = DOMPurify.sanitize(input);
    }

    parseDOM(dom: (value: DOMMessageParser) => DOMMessageParser) {
        this.input = dom(new DOMMessageParser(this.input)).build();

        return this;
    }

    parseForMarkdown() {
        this.input = marked.parse(this.input);
        this.input = DOMPurify.sanitize(this.input);

        return this;
    }

    public build() {
        return (<Box dangerouslySetInnerHTML={{ __html: this.input }}></Box>);
    }
}

export default IncomingMessageParser;