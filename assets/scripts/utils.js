async function addToCart(product_id, quantity) {

  const cartItem = {
    product_id: Number(product_id),
    quantity: Number(quantity),
  };

  const token = getBearerToken();

  fetch("http://127.0.0.1:3000/add_to_cart", {
    method: "POST",
    body: JSON.stringify(cartItem),
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${token}`,
    }
  })
    .then((response) => {
      if (response.status == 400 || response.status == 401) {
        window.location.replace("/login.html");
      } else {
        response.text().then((responseMessage) => {
          window.alert(responseMessage);
        })
      }
    })
    .catch((error) => {
      console.error("Error:", error);
    })
}

function getBearerToken() {
  const cookie = `; ${document.cookie}`;
  const parts = cookie.split("; access_token=");
  let token;
  if (parts.length === 2) {
    token = parts.pop().split(';').shift();
  } else {
    window.location.replace("/login.html");
  }
  token = token.replace(/^\[|\]$/g, '');  // trim the leading and trailing brackets from the token

  return token;
}