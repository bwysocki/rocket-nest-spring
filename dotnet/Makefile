aot:
	dotnet publish -c release -o publish -p:PublishAot=true -p:IlcOptimizationPreference=speed --use-current-runtime

jit:
	dotnet publish -c release -o publish -p:PublishSingleFile=true -p:PublishTrimmed=true --use-current-runtime