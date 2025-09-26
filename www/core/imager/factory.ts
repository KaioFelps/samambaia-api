import { HotelsCode, hotelsConfig, type THotelsCode } from "@/config/hotels";
import { Imager } from "./imager";
import { HabbletImagerProvider, HabbliveImagerProvider, type ImagerProvider } from "./providers";

export abstract class ImagerFactory {
  public static create(hotel: THotelsCode): Imager {
    let provider: ImagerProvider;

    switch (hotel) {
      case HotelsCode.Habblet:
        provider = new HabbletImagerProvider(
          hotelsConfig[hotel].imagerUrl,
          hotelsConfig[hotel].apiBaseUrl,
        );
        break;

      case HotelsCode.Habblive:
        provider = new HabbliveImagerProvider(hotelsConfig[hotel].imagerUrl);
        break;
    }

    return new Imager(provider);
  }
}
