import { Box } from "@mui/material";
import DOMPurify from "dompurify";
import { marked } from "marked";
import { createEmbeddedIFrame } from "./DOMHelper";

interface LinkReplacement {
    regex: RegExp;
    replacement: string;
    inline: boolean;
}

class DOMMessageParser {
    private document: Document;
    private replacementUrl: LinkReplacement[] = [
        { regex: /https:\/\/store\.steampowered\.com\/app\/([0-9]+)\/?.*/, replacement: 'steam://advertise/$1', inline: false },
        { regex: /https:\/\/www\.youtube\.com\/watch\?v=([a-zA-Z0-9_-]+)/, replacement: 'https://www.youtube.com/embed/$1', inline: true },
        { regex: /https:\/\/www\.twitch\.tv\/videos\/([0-9]+)/, replacement: 'https://player.twitch.tv/?video=$1&parent=' + (window.location.hostname), inline: true },
        { regex: /https:\/\/clips\.twitch\.tv\/([a-zA-Z0-9_-]+)/, replacement: 'https://clips.twitch.tv/embed?clip=$1&parent=' + (window.location.hostname), inline: true },
        { regex: /https:\/\/www\.twitch\.tv\/([a-zA-Z0-9_-]+)/, replacement: 'https://player.twitch.tv/?channel=$1&parent=' + (window.location.hostname), inline: true },
        { regex: /https:\/\/twitter\.com\/([a-zA-Z0-9_]+)\/status\/([0-9]+)/, replacement: 'https://twitframe.com/show?url=https://twitter.com/$1/status/$2', inline: true },
        { regex: /https:\/\/twitter\.com\/([a-zA-Z0-9_]+)/, replacement: 'https://twitframe.com/show?url=https://twitter.com/$1', inline: true },
        { regex: /https:\/\/giphy.com\/gifs\/.*-([a-zA-Z0-9_-]+)/, replacement: 'https://giphy.com/embed/$1', inline: true },
    ];

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
            let replaced = !this.replacementUrl.every(r => {
                if (e.getAttribute('href')?.match(r.regex)) {
                    let replaced = e.getAttribute('href')?.replace(r.regex, r.replacement) || '';

                    if (!r.inline) {
                        e.setAttribute('href', replaced);
                    } else {
                        const b = createEmbeddedIFrame(replaced);
                        e.parentElement?.replaceChild(b, e);
                    }
                    return false;
                }
                return true;
            });
            if (!replaced) e.setAttribute('target', '_blank');
        });

        return this;
    }

    build() {
        return this.document.body.innerHTML;
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