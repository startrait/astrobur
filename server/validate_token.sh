curl -XPOST "localhost:7777/validate-token" \
	-H 'Content-Type: application/json' \
	-d '
	{
		"token" : "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOm51bGwsImV4cCI6MTczNzcwNDM0MSwic3ViIjoiMiJ9.llhevDbE8yJS000hQ4X8men_gt9ZFjrgv5lzoKqBEYM"
	}
	'
