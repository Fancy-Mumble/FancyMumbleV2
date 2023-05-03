import { Box } from "@mui/material";
import DOMPurify from "dompurify";

class IncomingMessageParser {
    private document: Document;
    //private elements: Array<JSX.Element> = [];

    constructor(input: string) {
        let cleanMessage = DOMPurify.sanitize(input);
        const parser = new DOMParser();
        this.document = parser.parseFromString(cleanMessage, "text/html");
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

    public build() {
        return (<Box dangerouslySetInnerHTML={{__html: this.document.documentElement.innerHTML}}></Box>);
    }
}

export default IncomingMessageParser;