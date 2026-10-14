# T1 v2 cap50 rerun (doc 21), weak model (Gemini Flash-Lite) through the proxy

Build 720572915fc3c2be665fd6e3818ab312bdfb698d; proxy pin 8ce5135be68f0554e555a25e625b6f1890536e66; agent limit 900 s; interleaved F1 H1 F2 H2 F3 H3; all 6 runs: 0 × 503, 0 upstream transport errors, 0 client-side rows. No win claims.

Harness H1 and H3 stopped after 2 requests: the second model reply had 0 output tokens, and Harness ended the run as Succeeded with no edit. Oracle fail.

```
# counted runs: 6; oracle success 4 of 6; refused: 0
run	oracle_success	stop_reason	wall_ms	hold_ms	tool_calls	in_tok	out_tok	proxy_requests	proxy_input_tokens	proxy_output_tokens	steps_to_first_edit
T1-trackB-fx-isolated-cap50-rerun3-F1-20261014T194949	True	exit 0	64407	34494	8	57687	778	12	65459	843	5
T1-trackB-harness-isolated-cap50-rerun3-H1-20261014T195100	False	Succeeded	6045	4560	1	1289	10	2	1289	10	None
T1-trackB-fx-isolated-cap50-rerun3-F2-20261014T195113	True	exit 0	57314	43996	8	58086	664	12	65861	755	5
T1-trackB-harness-isolated-cap50-rerun3-H2-20261014T195217	True	Succeeded	60803	50171	12	27649	587	13	27649	587	6
T1-trackB-fx-isolated-cap50-rerun3-F3-20261014T195324	True	exit 0	55883	43900	8	57954	648	12	65725	766	5
T1-trackB-harness-isolated-cap50-rerun3-H3-20261014T195426	False	Succeeded	6256	4565	1	1289	10	2	1289	10	None
```
