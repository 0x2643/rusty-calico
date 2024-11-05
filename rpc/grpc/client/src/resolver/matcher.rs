use calico_grpc_core::protowire::{calicod_request, calicod_response, CalicodRequest, CalicodResponse};

pub(crate) trait Matcher<T> {
    fn is_matching(&self, response: T) -> bool;
}

impl Matcher<&calicod_response::Payload> for calicod_request::Payload {
    fn is_matching(&self, response: &calicod_response::Payload) -> bool {
        use calicod_request::Payload;
        match self {
            // TODO: implement for each payload variant supporting request/response pairing
            Payload::GetBlockRequest(ref request) => {
                if let calicod_response::Payload::GetBlockResponse(ref response) = response {
                    if let Some(block) = response.block.as_ref() {
                        if let Some(verbose_data) = block.verbose_data.as_ref() {
                            return verbose_data.hash == request.hash;
                        }
                        return true;
                    } else if let Some(error) = response.error.as_ref() {
                        // the response error message should contain the requested hash
                        return error.message.contains(request.hash.as_str());
                    }
                }
                false
            }

            _ => true,
        }
    }
}

impl Matcher<&CalicodResponse> for CalicodRequest {
    fn is_matching(&self, response: &CalicodResponse) -> bool {
        if let Some(ref response) = response.payload {
            if let Some(ref request) = self.payload {
                return request.is_matching(response);
            }
        }
        false
    }
}
