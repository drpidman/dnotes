import { css } from "@emotion/css";

export const WindowTop = css`
    position: fixed;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: .9rem 0;
    width: 100%;
    background-color: var(--primary-color);
    border-bottom: 1px dotted var(--third-color-shadow);
    z-index: 1;
`;

export const WindowTitle = css`
    padding: 0 1.5rem;
`;

export const WindowActions = css`
    display: flex;
    width: 60%;
    justify-content: space-between;
    align-items: center;
    font-weight: bold;
    padding: 0 1.5rem;
    font-size: 1.3rem;

    span {
        display: flex;
        align-items: center;
        cursor: pointer;
        padding: .2rem .2rem;
        border-radius: var(--border-radius-secondary);
        transition: 200ms all ease-in-out;
    }

    span:hover {
        background-color: var(--third-color);
        color: var(--primary-color);
    }
`;

export const SearchBar = css`
    width: 60%;

    input {
        width: 100%;
        background-color: var(--primary-color);
        border-radius: var(--border-radius-primary);
        border: 1px dotted var(--third-color-shadow);
        color: var(--third-color);
        font-size: 1.2rem;
        padding: .5rem 1.2rem;
        outline: none;


        ::placeholder {
            color: var(--third-color-shadow)
        }
    }
`;