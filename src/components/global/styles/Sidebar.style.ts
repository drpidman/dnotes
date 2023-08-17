import { css } from "@emotion/css";

export const SideBar = css`
    display: flex;
    height: 100%;
    width: -webkit-fill-max;
    margin: 0.8rem 1.2rem;
    grid-area: aside;
    flex-direction: column;
    border-right: .1px dotted var(--third-color-shadow);

    ul {
        list-style: none;
        padding: 1rem;
        column-gap: 1.5rem;
        gap: 1.2rem;
    }
    
    ul li {
        position: relative;
        gap: 1.5rem;
        margin-top: .9rem;
        display: flex;
        align-items: center;
        padding: 1rem 1.9rem;
        transition: 100ms all ease-in-out;
        border-radius: var(--border-radius-primary);
        font-size: 1.3rem;
        cursor: pointer;
    }

    ul li:hover {
        background-color: var(--primary-color);
        color: var(--orange);
    }
`