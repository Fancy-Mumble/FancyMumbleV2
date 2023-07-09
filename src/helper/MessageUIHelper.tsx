import { Container } from '@mui/material';
import parse, { DOMNode, HTMLReactParserOptions, domToReact } from 'html-react-parser';
import LightBoxImage from '../components/LightBoxImage';
import UrlPreview from '../components/UrlPreview';

export default class MessageUIHelper {
    private input: string;
    private loaded: (() => void) | undefined;

    constructor(input: string, loaded?: () => void) {
        this.input = input;
        this.loaded = loaded;
    }

    build() {
        const options: HTMLReactParserOptions = {
            replace: ({ name, attribs, children }: any) => {
                switch (name) {
                    case 'img':
                        return (<Container  >
                            <LightBoxImage src={attribs.src} />
                        </Container>);
                    case 'a':
                        return (<Container>
                            <UrlPreview href={attribs.href} onLoaded={this.loaded} />
                        </Container>)
                }
            },
        };

        return parse(this.input, options);
    }
}