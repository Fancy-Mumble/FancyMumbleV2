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

    parseCommands() {
        const commandRegex = /^@[A-Za-z]+/
        let foundCommand = this.message.match(commandRegex);
        if (!foundCommand) return this;

        //TODO: Move this to the backend
        switch (foundCommand[0]) {
            case "@dice":
                let diceRoll = Math.floor(Math.random() * 6) + 1;
                this.message = this.message.replace(commandRegex, "The dice rolled: " + diceRoll);
                break;
            case "@coin":
                let coinFlip = Math.floor(Math.random() * 2) + 1;
                this.message = this.message.replace(commandRegex, "Coin flip: " + (coinFlip === 1 ? "Heads" : "Tails"));
                break;
        }

        return this;
    }

    public build() {
        return this.message;
    }
}

export default OutgoingMessageParser;