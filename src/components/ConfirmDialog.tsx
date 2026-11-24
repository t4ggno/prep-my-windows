import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@/components/ui/dialog";

export interface ConfirmRequest {
  title: string;
  description: string;
  confirmLabel: string;
  onConfirm: () => void;
}

export function ConfirmDialog({
  request,
  onClose,
}: {
  request: ConfirmRequest | null;
  onClose: () => void;
}) {
  return (
    <Dialog open={request !== null} onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="w-[min(440px,calc(100vw-32px))]">
        {request && (
          <div className="grid gap-5 p-6">
            <div className="grid gap-2 pr-6">
              <DialogTitle>{request.title}</DialogTitle>
              <DialogDescription>{request.description}</DialogDescription>
            </div>
            <div className="flex justify-end gap-2">
              <Button variant="secondary" onClick={onClose}>
                Cancel
              </Button>
              <Button
                variant="destructive"
                autoFocus
                onClick={() => {
                  request.onConfirm();
                  onClose();
                }}
              >
                {request.confirmLabel}
              </Button>
            </div>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
