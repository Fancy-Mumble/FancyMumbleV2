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
        const images = Array.from(this.document.querySelectorAll('img')).map(img => img.src);

        (<div key={Math.random()}>
            {images.map(e =>
                <img key={e} src={e} style={{ maxWidth: '100%' }} />
            )}
        </div>);

        return this;
    }

    public build() {
        return (<Box dangerouslySetInnerHTML={{__html: this.document.documentElement.innerHTML}}></Box>);
    }
}

export default IncomingMessageParser;