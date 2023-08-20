import { css } from '@emotion/css';

export const SideBar = css`
	display: flex;
	height: 100vh;
	width: 50%;
	margin: 0.8rem 1.2rem;
	grid-area: aside;
	flex-direction: column;
	border-right: 0.1px dotted var(--third-color-shadow);

	ul {
		margin-top: 1.1rem;
		list-style: none;
		padding: 1rem;
	}

	ul li {
		position: relative;
		gap: 1.5rem;
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

	span {
		border-left: 1px dotted var(--third-color-shadow);
		margin-left: 2.5rem;
	}
`;
