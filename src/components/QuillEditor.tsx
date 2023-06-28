import React, { useEffect, useRef, useState } from 'react';
import Quill, { TextChangeHandler } from 'quill';
import 'quill/dist/quill.snow.css';

interface QuillEditorProps {
    theme?: 'bubble' | 'snow' | string;
    placeholder?: string;
    readOnly?: boolean;
    bounds?: string;
    debug?: 'error' | 'warn' | 'log' | boolean;
    formats?: string[];
    modules?: any;
    scrollingContainer?: string | HTMLElement | undefined;
    style?: React.CSSProperties;
    onKeyDown?: (this: HTMLDivElement, ev: KeyboardEvent) => any;
    onChange?: (content: string) => void;
    onPaste?: (this: HTMLDivElement, ev: ClipboardEvent) => any;
    multiline?: boolean;
    value?: string;
}

export const QuillEditor: React.FC<QuillEditorProps> = ({
    theme = 'snow',
    placeholder = 'Compose an epic...',
    readOnly = false,
    bounds = 'document.body',
    debug = 'warn',
    formats = [],
    modules = {},
    scrollingContainer = undefined,
    style = {},
    onKeyDown,
    onChange,
    onPaste,
    multiline = false,
    value = ''
}) => {
    const editorRef = useRef<HTMLDivElement | null>(null);
    const quillRef = useRef(); // This will hold the Quill instance
    const [toolbarVisible, setToolbarVisible] = useState(multiline);
    let quill: Quill | undefined = undefined;
    let textChangeListener: TextChangeHandler | undefined = undefined;

    // This runs once on component mount
    useEffect(() => {
        if (editorRef.current) {
            quill = new Quill(editorRef.current, {
                theme,
                placeholder,
                readOnly,
                bounds,
                debug,
                formats,
                modules,
                scrollingContainer
            });

            // Set the initial value
            quill.root.innerHTML = value;

            // Add event listeners
            if (onKeyDown)
                quill.root.addEventListener('keydown', onKeyDown);
            if (onPaste)
                quill.root.addEventListener('paste', onPaste);

            if (onChange) {
                textChangeListener = () => {
                    onChange(quill?.root.innerHTML || '');
                }

                quill.on('text-change', textChangeListener);
            }


            // Check for multiline to show/hide toolbar
            if (multiline) {
                quill.on('text-change', () => {
                    const text = quill?.getText();
                    const lineCount = (text?.match(/\n/g) || []).length;
                    setToolbarVisible(lineCount > 1);
                });
            }
        }

        // Clean up on component unmount
        return () => {
            if (quill) {
                if (onKeyDown) {
                    quill.root.removeEventListener('keydown', onKeyDown);
                }

                if (textChangeListener) {
                    quill?.off('text-change', textChangeListener);
                }

                if (onPaste) {
                    quill.root.removeEventListener('paste', onPaste);
                }
            }
        };
    }, []); // Empty array means this effect runs once on mount and clean up on unmount


    useEffect(() => {
        if (editorRef.current) {


            /*if (quillRef) {
                quillRef.current = quill;
            }*/
        }
    }, [theme, placeholder, readOnly, bounds, debug, formats, modules, scrollingContainer, quillRef]);

    return <div className="quill-wrapper" style={style}>
        {toolbarVisible && <div id="toolbar">/* Add toolbar contents here */</div>}
        <div ref={editorRef} />
    </div>;
};