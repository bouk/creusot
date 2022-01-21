fn uses_closure() {
  let y = true;
  let x = (|| { y })();
}