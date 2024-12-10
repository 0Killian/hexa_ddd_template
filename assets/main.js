
// we do this because our API return errors in the format { "data": JsonError, "html": HtmlError }
// so we intercept the response so that htmx will display the html inside
htmx.on('htmx:afterRequest', function (event) {
  const target = event.detail.target;
  const { response, status } = event.detail.xhr;

  if (status >= 400) {
    const { html, data } = JSON.parse(response);
    if (data) console.error(data);
    if (target && html) target.innerHTML = html;
  } else if (target && response) {
    target.innerHTML = response;
  }

  // after intercepting the request, we need to tell htmx to process its attribute in the new html
  htmx.process(target);
});

// Add Accept: text/html header
htmx.on('htmx:configRequest', function(evt) {
    evt.detail.headers['Accept'] = 'text/html';
});
