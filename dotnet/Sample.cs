using System.Text.Json.Serialization;

namespace RocketNestSpring;

public record Sample(
    string Fruit,
    string Size,
    string Color,
    string LongDesc,
    decimal Price,
    string? Extra);

[JsonSerializable(typeof(Sample))]
[JsonSourceGenerationOptions(PropertyNamingPolicy = JsonKnownNamingPolicy.CamelCase)]
public partial class SampleJsonContext : JsonSerializerContext { }