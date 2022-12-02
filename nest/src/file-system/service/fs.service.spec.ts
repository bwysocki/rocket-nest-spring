import { Test, TestingModule } from '@nestjs/testing';
import { FSService } from './fs.service';

describe('FSService', () => {
  let service: FSService;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [FSService],
    }).compile();

    service = module.get<FSService>(FSService);
  });

  it('should be defined', () => {
    expect(service).toBeDefined();
  });
});
