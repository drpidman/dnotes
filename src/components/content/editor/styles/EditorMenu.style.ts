import { css } from '@emotion/css';

export const EditorMenu = css`
	width: 100%;
	display: flex;
	align-items: center;
	height: min-content;
	padding: 1.5rem;
	margin-top: 1.2rem;

	span[role='button'] {
		display: flex;
		align-items: center;
		cursor: pointer;
		padding: 0.2rem 0.2rem;
		border-radius: var(--border-radius-secondary);
		transition: 200ms all ease-in-out;
	}

	span[role='button']:hover {
		background-color: var(--third-color);
		color: var(--primary-color);
	}
`;
