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

class MessageParser {
    private input: string;

    constructor(input: string) {
        this.input = DOMPurify.sanitize(input);
    }

    parseDOM(dom: (value: DOMMessageParser) => DOMMessageParser) {
        this.input = dom(new DOMMessageParser(this.input)).build();

        return this;
    }

    parseLinks() {
        const regex = /(?<!\S)((?:https?:\/\/)(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-zA-Z0-9()]{2,20}\b[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)/;
        this.input = this.input.replace(regex, '<a href="$1" target="_blank">$1</a>');

        return this;
    }

    parseCommands() {
        const commandRegex = /^@[A-Za-z]+/
        let foundCommand = this.input.match(commandRegex);
        if (!foundCommand) return this;

        //TODO: Move this to the backend
        switch (foundCommand[0]) {
            case "@dice":
                let diceRoll = Math.floor(Math.random() * 6) + 1;
                this.input = this.input.replace(commandRegex, "The dice rolled: \n # " + diceRoll);
                break;
            case "@coin":
                let coinFlip = Math.floor(Math.random() * 2) + 1;
                this.input = this.input.replace(commandRegex, "Coin flip: " + (coinFlip === 1 ? "Heads" : "Tails"));
                break;
        }

        return this;
    }

    parseMarkdown() {
        this.input = marked.parse(this.input);
        this.input = DOMPurify.sanitize(this.input);

        return this;
    }

    public build() {
        return (<Box dangerouslySetInnerHTML={{ __html: this.input }}></Box>);
    }

    public buildString() {
        return this.input;
    }
}

export default MessageParser;