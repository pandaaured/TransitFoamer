export interface UniqueSequence {
  sequence_idx: number
  stops: [string, string, number][]  // [stop_id, stop_name, sequence_number]
}
