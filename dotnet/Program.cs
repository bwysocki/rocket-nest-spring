using System.Text.Json;
using RocketNestSpring;

var builder = WebApplication.CreateBuilder(args);
builder.Services.AddLogging();

var host = builder.Build();
var logger = host.Services.GetRequiredService<ILogger<Program>>();

// Using context over directly injecting 'extra' with async ([FromQuery] string extra) => {...}
// because doing so requires dynamic code generation to produce RequestDelegate for us,
// which is unsupported in Native AOT (.NET's analogue to Graal VM which I assume is AOT).
// However, .NET is different to Java because there is little profit in using AOT
// in classic back-end scenarios. For easy deployment just see 'jit' makefile target.
host.MapGet("/spring-filesystem-read", async context =>
{
    try
    {
        var file = await File.ReadAllBytesAsync("Resources/sample.json");
        var extra = context.Request.Query["extra"][0]!;
        var sample = JsonSerializer.Deserialize(file, SampleJsonContext.Default.Sample)!
            with { Extra = extra };
        // Using .Bytes over .Json because the latter needs dynamic code generation unavailable, see above
        await Results
            .Bytes(
                JsonSerializer.SerializeToUtf8Bytes(sample, SampleJsonContext.Default.Sample),
                "application/json")
            .ExecuteAsync(context);
    }
    catch (Exception e)
    {
        logger.LogError(e, "Can not read response");

        await Results
            .NotFound("Unable to find resource")
            .ExecuteAsync(context);
    }
});

host.Run("http://localhost:8080");
