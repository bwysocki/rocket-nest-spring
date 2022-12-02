import { Test, TestingModule } from '@nestjs/testing';
import { FSController } from './fs.controller';

describe('FSController', () => {
  let controller: FSController;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [FSController],
    }).compile();

    controller = module.get<FSController>(FSController);
  });

  it('should be defined', () => {
    expect(controller).toBeDefined();
  });
});
