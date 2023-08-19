import { css } from '@emotion/css';

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
`;
