import { Logo } from './logo';
import './styles/base.scss';

export function App() {
	return (
		<>
			<Logo />
			<p className="text-indigo-900">Hello Vite + Preact!</p>
			<p>
				<a
					class="link"
					href="https://preactjs.com/"
					target="_blank"
					rel="noopener noreferrer"
				>
					Learn Preact!
				</a>
			</p>
		</>
	);
}
