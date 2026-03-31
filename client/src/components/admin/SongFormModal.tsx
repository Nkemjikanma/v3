import { useState, useEffect } from "react";
import { useAddSong, useUpdateSong } from "../../lib/hooks/useSongs";
import type { Song, Instrument } from "../../lib/types";

type SongFormModalProps = {
	isOpen: boolean;
	onClose: () => void;
	song?: Song;
};

export default function SongFormModal({ isOpen, onClose, song }: SongFormModalProps) {
	const [title, setTitle] = useState("");
	const [artist, setArtist] = useState("");
	const [instrument, setInstrument] = useState<Instrument>("guitar");
	const [startedLearningAt, setStartedLearningAt] = useState("");
	const [notes, setNotes] = useState("");
	const [error, setError] = useState("");

	const addSong = useAddSong();
	const updateSong = useUpdateSong();

	const isPending = addSong.isPending || updateSong.isPending;

	useEffect(() => {
		if (song) {
			setTitle(song.title);
			setArtist(song.artist);
			setInstrument(song.instrument);
			setStartedLearningAt(song.started_learning_at);
			setNotes(song.notes ?? "");
		} else {
			setTitle("");
			setArtist("");
			setInstrument("guitar");
			setStartedLearningAt("");
			setNotes("");
		}
		setError("");
	}, [song, isOpen]);

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		setError("");

		if (!title.trim() || !artist.trim()) {
			setError("Title and artist are required");
			return;
		}

		if (!startedLearningAt) {
			setError("Start date is required");
			return;
		}

		const onSuccess = () => onClose();
		const onError = (err: Error) => setError(err.message);

		if (song) {
			updateSong.mutate(
				{
					id: song.id,
					song: {
						title,
						artist,
						instrument,
						notes: notes || undefined,
						started_learning_at: startedLearningAt,
					},
				},
				{ onSuccess, onError }
			);
		} else {
			addSong.mutate(
				{
					title,
					artist,
					instrument,
					started_learning_at: startedLearningAt,
					notes: notes || undefined,
				},
				{ onSuccess, onError }
			);
		}
	};

	if (!isOpen) return null;

	return (
		<div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
			<form
				onSubmit={handleSubmit}
				className="bg-white p-6 rounded space-y-4 w-full max-w-md"
			>
				<h2 className="text-xl font-outfitSemiBold">
					{song ? "Edit Song" : "Add Song"}
				</h2>

				<div className="space-y-1">
					<label className="text-sm">Title</label>
					<input
						type="text"
						value={title}
						onChange={(e) => setTitle(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					/>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Artist</label>
					<input
						type="text"
						value={artist}
						onChange={(e) => setArtist(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					/>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Instrument</label>
					<select
						value={instrument}
						onChange={(e) => setInstrument(e.target.value as Instrument)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					>
						<option value="guitar">Guitar</option>
						<option value="piano">Piano</option>
						<option value="both">Both</option>
					</select>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Started Learning</label>
					<input
						type="date"
						value={startedLearningAt}
						onChange={(e) => setStartedLearningAt(e.target.value)}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black"
					/>
				</div>

				<div className="space-y-1">
					<label className="text-sm">Notes (optional)</label>
					<textarea
						value={notes}
						onChange={(e) => setNotes(e.target.value)}
						rows={3}
						className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none
  focus:border-black resize-none"
					/>
				</div>

				{error && <p className="text-red-500 text-sm">{error}</p>}

				<div className="flex gap-3 justify-end">
					<button
						type="button"
						onClick={onClose}
						className="px-4 py-2 border border-gray-300 rounded text-sm"
					>
						Cancel
					</button>
					<button
						type="submit"
						disabled={isPending}
						className="px-4 py-2 bg-black text-white rounded text-sm disabled:opacity-50"
					>
						{isPending ? "Saving..." : song ? "Update" : "Add"}
					</button>
				</div>
			</form>
		</div>
	);
}
