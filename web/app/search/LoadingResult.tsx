// LoadingResult.tsx
import React from "react";
import BoidsCanvas from "@/components/animations/boids/BoidsCanvas"
import PingPong from "@/components/animations/pingpong/PingPong";

const LoadingResult: React.FC = () => {
    return (
        <>
            <div className="col-lg-8 col-xl-7">
                <div className="container-xl container">
                    <div>
                        <h6 className="text-uppercase heading">Answer | gpt-3.5 Model</h6>
                        <div className="position-relative mt-4 colContainer">
                            <PingPong />
                        </div>
                    </div>
                    <div className="position-relative mt-4 colContainer" />
                </div>
            </div>
        </>
    );
};

export default LoadingResult;
