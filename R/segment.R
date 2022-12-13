#' Segment sentences into tokens
#'
#' @param sentence character vector.
#' @param model path to a model file if you have.
#' @return a list.
#' @export
segment <- function(sentence, model = NULL) {
  stopifnot(
    is.character(sentence)
  )
  if (missing(model)) {
    model <- system.file("model/bccwj-suw+unidic+tag.model.zst", package = "segmntr")
  }
  nm <- names(sentence)
  if (is.null(nm)) {
    nm <- seq_along(sentence)
  }
  ret <- vaporetto(
    unname(sentence),
    model = model
  )
  names(ret) <- nm
  return(ret)
}
