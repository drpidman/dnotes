import { css } from '@emotion/css';

export const Note = css`
	position: relative;
	margin: 0.5rem 0;
	border-radius: var(--border-radius-primary);
	padding: 1.5rem;
	display: flex;
	flex-direction: column;

	width: 95%;
	transition: all 200ms ease-in-out;

	cursor: var(--cursor-type);

	:before {
		content: '';
		position: absolute;
		width: 0;
		height: 100%;
		transition: all 200ms ease-in-out;
		right: 0;
		top: 0;
		background-color: var(--bars-accent-color);
	}

	:after {
		content: '';
		position: absolute;
		width: 0;
		height: 100%;
		transition: all 200ms ease-in-out;
		left: 0;
		top: 0;
		background-color: var(--bars-accent-color);
		z-index: -1;
	}

	:hover::before,
	:hover::after {
		width: 1%;
	}

	:active {
		background-color: var(--enabled-color);
	}
`;

export const NoteHead = css`
	width: 100%;
`;

export const NoteTags = css`
	width: 50%;
	display: flex;
	flex-direction: row;
	margin-top: 0.5rem;
	gap: 0.4rem;
	flex-wrap: wrap;
	flex-grow: 300px;

	span {
		font-size: 0.9rem;
		display: flex;
		align-items: center;
		border-radius: var(--border-radius-primary);
		padding: 0.2rem 1.1rem;
		background-color: var(--third-color);
		color: var(--primary-color);
	}
`;

export const NoteBody = css`
	margin-top: 0.5rem;
`;
