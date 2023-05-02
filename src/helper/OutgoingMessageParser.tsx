import DOMPurify from "dompurify";

class OutgoingMessageParser {
    private message: string;

    constructor(input: string) {
        this.message = input;
    }

    parseLinks() {
        const regex = /(?<!\S)((?:https?:\/\/)?(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-zA-Z0-9()]{2,20}\b[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)/;
        this.message = this.message.replace(regex, '<a href="$1" target="_blank">$1</a>');
        console.log(this.message);

        return this;
    }

    public build() {
        return this.message;
    }
}

export default OutgoingMessageParser;