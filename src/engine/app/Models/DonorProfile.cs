using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace app.Models;

public class DonorProfile
{
    [Key] 
    [DatabaseGenerated(DatabaseGeneratedOption.Identity)]
    public Guid Id { get; set; } 
    public string? Nome { get; init; }

    [EmailAddress(ErrorMessage = "Email inválido.")]
    public string? Email { get; init; }

    [RegularExpression(@"^\d{2}:\d{2}$", ErrorMessage = "Formato de horário inválido (HH:MM).")]
    public string? Horario { get; init; }

    public bool Doador { get; init; }

    public string? LocalDeCobranca { get; init; }

    public string? Morada { get; init; }

    public string? Freguesia { get; init; }

    public string? Concelho { get; init; }

    [RegularExpression(
        @"^\d{4}-\d{3}$",
        ErrorMessage = "Formato de código postal inválido (0000-000)."
    )]
    public string? CodigoPostal { get; init; }

    [StringLength(9, MinimumLength = 9, ErrorMessage = "Telefone residencial deve ter 9 dígitos.")]
    [RegularExpression(@"^\d{9}$", ErrorMessage = "Apenas números são permitidos.")]
    public string? TelResidencial { get; init; }

    [StringLength(9, MinimumLength = 9, ErrorMessage = "Telefone de trabalho deve ter 9 dígitos.")]
    [RegularExpression(@"^\d{9}$", ErrorMessage = "Apenas números são permitidos.")]
    public string? TelTrabalho { get; init; }

    [Required(ErrorMessage = "Telemóvel é obrigatório.")]
    [StringLength(9, MinimumLength = 9, ErrorMessage = "Telemóvel deve ter 9 dígitos.")]
    [RegularExpression(@"^\d{9}$", ErrorMessage = "Apenas números são permitidos.")]
    public string? Telemovel { get; init; }

    public string? Observacoes { get; init; }
}
