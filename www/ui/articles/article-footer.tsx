import { Sprite } from "@/components/sprite";
import { FaceGesture, Imager } from "@/utils/imager";

type Props = {
  authorNickname: string;
  publishmentDate: string | Date;
};

export function ArticleFooter({ authorNickname, publishmentDate }: Props) {
  return (
    <section
      id="article-footer-container"
      className="card py-0 flex items-center justify-between mb-2">
      <div id="article-author-container" className="flex gap-1 items-center">
        <div
          style={{
            background: `url(${Imager.getUserImage(authorNickname, {
              gesture: FaceGesture.smile,
              direction: "2",
              head_direction: "3",
              size: "m",
              img_format: "png",
            })}) no-repeat calc((90px - 64px) / 2 * -1) -26px`,
          }}
          className="pixelated w-16 h-[60px] my-0.5"
        />
        <div className="flex flex-col justify-center items-start text-sm text-gray-700">
          <span className="flex gap-0.5 items-center">
            <Sprite width={16} height={17} x={-139} y={-64} />
            Escrito por <strong>{authorNickname}</strong>
          </span>
          <span className="flex gap-0.5 items-center">
            <Sprite width={16} height={17} x={-128} y={-95} />
            Publicado em <strong>{new Date(publishmentDate).toLocaleDateString("pt-BR")}</strong>
          </span>
        </div>
      </div>
      <div id="article-actions-container" className="py-3 flex items-center justify-end gap-2">
        {/* TODO: make forms work in order to enable this button */}
        {/* <PublicButton.Default variant="success" size="lg">
              <Sprite width={18} height={13} x={-94} y={-96} />
              Formulário
            </PublicButton.Default> */}

        {/* TODO: make stars work in order to enable this button */}
        {/* <PublicButton.Default variant="yellow" size="lg">
              <Sprite width={16} height={14} x={-112} y={-64} /> 1
            </PublicButton.Default> */}
      </div>
    </section>
  );
}
