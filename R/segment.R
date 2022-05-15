#' Segment sentences into tokens
#'
#' @param sentence character vector.
#' @return a list.
#' @export
segment <- function(sentence) {
  vaporetto(unname(sentence))
}
