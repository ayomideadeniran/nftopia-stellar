
import { ApiProperty } from '@nestjs/swagger';

export class StellarNftDto {
    @ApiProperty({ description: 'Stellar Contract ID (C...)' })
    contractId: string;

    @ApiProperty({ description: 'Token ID (uint256 or string)' })
    tokenId: string;

    @ApiProperty({ description: 'Owner Account ID (G...)' })
    owner: string;

    @ApiProperty({ required: false, description: 'IPFS or HTTP URI' })
    metadataUri?: string;

    @ApiProperty({ required: false })
    name?: string;

    @ApiProperty({ required: false })
    description?: string;

    @ApiProperty({ required: false })
    image?: string;
}
