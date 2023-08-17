import { css } from "@emotion/css";

export const Content = css`
    grid-area: content;
    width: 100%;
    display: flex;
    align-items: center;
    flex-direction: column;
    padding: 1.9rem 0 0 0;
    height: -webkit-fill-available;
    overflow-y: scroll;

    ::-webkit-scrollbar {
        width: 0;
    }
`

export const Note = css`
    position: relative;
    margin: .5rem 0;
    border-radius: var(--border-radius-primary);
    padding: 1.5rem;
    display: flex;
    flex-direction: column;

    width: 95%;
    transition: all 200ms ease-in-out;

    cursor: pointer;

    :before {
        content: "";
        position: absolute;
        width: 0;
        height: 100%;
        transition: all 200ms ease-in-out;
        right: 0;
        top: 0;
        background-color: var(--orange);
    }

    :after {
        content: "";
        position: absolute;
        width: 0;
        height: 100%;
        transition: all 200ms ease-in-out;
        left: 0;
        top: 0;
        background-color: var(--orange);
        z-index: -1;
    }

    :hover::before {
        width: 1%;
    }

    :hover::after {
        width: 1%;
    }
`

export const NoteHead = css`
    width: 100%;
`

export const NoteTags = css`
    width: 50%;
    display: flex;
    flex-direction: row;
    margin-top: .5rem;
    gap: .4rem;
    flex-wrap: wrap;
    flex-grow: 300px;

    span {
        font-size: .9rem;
        display: flex;
        align-items: center;
        border-radius: var(--border-radius-primary);
        padding: .2rem 1.1rem;
        background-color: var(--third-color);
        color: var(--primary-color);
    }
`

export const NoteBody = css`
    margin-top: .5rem;
`

