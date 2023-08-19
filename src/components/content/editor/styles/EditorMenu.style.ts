import { css } from "@emotion/css";

export const EditorMenu = css`
    width: 100%;
    display: flex;
    align-items: center;
    height: min-content;
    padding: 1.5rem;

    span[role="button"] {
        display: flex;
        align-items: center;
        cursor: pointer;
        padding: .2rem .2rem;
        border-radius: var(--border-radius-secondary);
        transition: 200ms all ease-in-out;
    }

    span[role="button"]:hover {
        background-color: var(--third-color);
        color: var(--primary-color);
    }
`