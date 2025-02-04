import sys
import re

with open(sys.argv[1], "r") as file:
    without_ampersand = re.sub('&', '&amp;', file.read())
    without_quotes = re.sub('\"', '&quot;', without_ampersand)
    without_single_quotes = re.sub('\'', '&#39;', without_quotes)
    without_lt = re.sub('<', '&lt;', without_single_quotes)
    without_gt = re.sub('>', '&gt;', without_lt)
    print(f'<pre id=\"codeblock\"><code><!-- beautify ignore:start -->{without_gt}<!-- beautify ignore:end --></code></pre>')
