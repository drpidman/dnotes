import { css } from '@emotion/css';

export const ButtonPrimaryOutline = css`
	position: relative;
	background-color: transparent;
	border-radius: var(--border-radius-primary);
	padding: 0.9rem 0.5rem;
	border: 0.5px dashed var(--third-color);
	cursor: pointer;
	font-size: 1.2rem;
	font-weight: bold;
	color: var(--third-color);

	transition: all 200ms ease-in-out;

	:hover {
		background-color: var(--third-color);
		color: var(--primary-color);
	}

	:after {
		content: '';
		display: block;
		position: absolute;
		border-radius: var(--border-radius-primary);
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		opacity: 0;
		transition: all 0.5s;
		box-shadow: 0 0 14px 14px var(--third-color);
	}

	:active:after {
		box-shadow: 0 0 0 0 var(--third-color);
		position: absolute;
		border-radius: var(--border-radius-primary);
		left: 0;
		top: 0;
		opacity: 1;
		transition: 0s;
	}
`;
