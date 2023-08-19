import { css } from '@emotion/css';

export const EditorContainer = css`
	display: flex;
	justify-content: center;
	background-color: transparent;
	padding: 0 1.5rem;
	margin-bottom: 0.5rem;
	overflow-y: auto;
`;

export const TextEditor = css`
	width: 85%;
	height: -webkit-fill-avaliable;
	transition: all 200ms ease-in-out;


	div.milkdown div {
		height: 100vh;
		width: 100%;
		outline: none;

		color: var(--third-color);
		padding: 0 1.5rem;

		ul {
			padding-left: 0.9rem;
		}

		h1 {
			font-size: 1.6rem;
		}

		p {
			margin: 0.6rem 0;
		}

		blockquote {
			padding-left: 1.4rem;
			margin: 0.5rem;
			position: relative;

			::before {
				content: '';
				position: absolute;
				width: 0.5%;
				height: 100%;
				background-color: var(--orange);
				top: 0;
				left: 0;
			}
		}

		pre {
			padding: 1.1rem;
			background-color: var(--code-block-bg);
			border-radius: var(--border-radius-primary);
		}

		hr {
			height: 0.5px;
			margin: 1.2rem 0;
			border: 0.6px dotted var(--third-color-shadow);
		}
	}

	div.milkdown div > *:hover,
	div.milkdown div > *:active,
	div.milkdown div > *:focus-visible {
		background-color: var(--third-color-shadow);
	}

	div.milkdown div > pre:hover {
		background-color: var(--code-block-bg);
	}

`;
