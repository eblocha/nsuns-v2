import { useNavigate, useParams } from "@solidjs/router";

export const useNavigateToProfileHome = () => {
  const params = useParams<{ profileId?: string }>();
  const navigate = useNavigate();

  return (profileId?: string) => {
    const id = profileId || params.profileId;
    navigate(id ? `/profile/${id}` : "/");
  };
};
