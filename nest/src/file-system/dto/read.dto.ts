import { IsString, Length } from 'class-validator'

export class ReadParamDto {
  @IsString()
  @Length(1, 100)
  extra: string;
}
