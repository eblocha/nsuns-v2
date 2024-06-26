import { useNavigate, useParams } from "@solidjs/router";

export const useNavigateToLogin = () => {
  const navigate = useNavigate();
  return () => navigate("/login");
};

export const useNavigateToProfileHome = () => {
  const params = useParams<{ profileId?: string }>();
  const navigate = useNavigate();

  return (profileId?: string) => {
    const id = profileId || params.profileId;
    navigate(id ? `/profile/${id}` : "/");
  };
};

export const useNavigateToNewProgram = () => {
  const params = useParams<{ profileId?: string }>();
  const navigate = useNavigate();

  return () => {
    if (params.profileId) {
      navigate(`/profile/${params.profileId}/program/new`);
    }
  };
};

export const useNavigateToProgram = () => {
  const params = useParams<{ profileId?: string; programId?: string }>();
  const navigate = useNavigate();

  return (programId?: string) => {
    if (params.profileId) {
      const id = programId || params.programId;
      if (id) {
        navigate(`/profile/${params.profileId}/program/${id}`);
      }
    }
  };
};

export const useSwitchProfileInRunner = () => {
  const params = useParams<{ programId?: string }>();
  const navigate = useNavigate();

  return (profileId: string) => {
    if (params.programId) {
      navigate(`/profile/${profileId}/program/${params.programId}/run`);
    }
  };
};
