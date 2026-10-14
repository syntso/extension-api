const note = { project: null, path: null, base: null };

synt.on("open", (opened) => {
  Object.assign(note, opened);
  document.getElementById("text").value = opened.text;
});

document.getElementById("save").addEventListener("click", async () => {
  const text = document.getElementById("text").value;
  const saved = await synt.call({ ...note, text });

  note.base = saved.revision;
});
