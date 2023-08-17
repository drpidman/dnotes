import { css } from '@emotion/css';

export const GridLayout = css`
	display: grid;
	grid-template-areas:
		'aside aside content content'
		'aside aside content content';
	overflow: hidden;

	gap: 1.5rem;
`;
