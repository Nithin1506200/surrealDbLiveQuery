enum Session {
  TOKEN = "token",
}

function getLoginToken() {
  return sessionStorage.getItem(Session.TOKEN);
}
function setLoginToken(token: string) {
  return sessionStorage.setItem(Session.TOKEN, token);
}
const sessionStorageUtils = {
  getLoginToken,
  setLoginToken,
};
export default sessionStorageUtils;
