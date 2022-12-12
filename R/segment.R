#' Segment sentences into tokens
#'
#' @param sentence character vector.
#' @param model path to a model file if you have.
#' @return a list.
#' @export
segment <- function(sentence, model = NULL) {
  if (missing(model)) {
    model <- system.file("model/bccwj-suw+unidic+tag.model.zst", package = "segmntr")
  }
  ret <- vaporetto(
    unname(sentence),
    model = model
  )
  lapply(ret, function(el) unlist(strsplit(el, " ", fixed = TRUE)))
}
