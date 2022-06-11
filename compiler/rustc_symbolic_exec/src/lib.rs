//! This query borrow-checks the MIR to (further) ensure it is not broken.

#![allow(rustc::potential_query_instability)]
#![feature(box_patterns)]
#![feature(crate_visibility_modifier)]
#![feature(let_chains)]
#![feature(let_else)]
#![feature(min_specialization)]
#![feature(never_type)]
#![feature(stmt_expr_attributes)]
#![feature(trusted_step)]
#![feature(try_blocks)]
#![recursion_limit = "256"]

use tracing::debug;

use rustc_data_structures::graph::WithNumNodes;
use rustc_data_structures::graph::WithStartNode;
use rustc_data_structures::graph::WithSuccessors;
use smallvec::SmallVec;

use rustc_data_structures::vec_map::VecMap;
use rustc_hir::def_id::LocalDefId;
// use rustc_index::vec::IndexVec;
// use rustc_infer::infer::{TyCtxtInferExt};
use rustc_middle::mir::{Body};
use rustc_middle::mir::BorrowCheckResult;
use rustc_middle::ty::query::Providers;
use rustc_middle::ty::{self, TyCtxt};

pub fn provide(providers: &mut Providers) {
    *providers = Providers {
        mir_symbolic_exec: |tcx, did| {
            if let Some(def) = ty::WithOptConstParam::try_lookup(did, tcx) {
                tcx.mir_symbolic_exec_const_arg(def)
            } else {
                mir_symbolic_exec(tcx, ty::WithOptConstParam::unknown(did))
            }
        },
        mir_symbolic_exec_const_arg: |tcx, (did, param_did)| {
            mir_symbolic_exec(tcx, ty::WithOptConstParam { did, const_param_did: Some(param_did) })
        },
        ..*providers
    };
}



fn mir_symbolic_exec<'tcx>(
    tcx: TyCtxt<'tcx>,
    _def: ty::WithOptConstParam<LocalDefId>,
) -> &'tcx BorrowCheckResult<'tcx> {
    let (_input_body, _promoted) = tcx.mir_promoted(_def);

    let _to_print_input_body: &Body<'_> = &_input_body.borrow();
    debug!("Number of Nodes: {}", _to_print_input_body.num_nodes());
    _to_print_input_body.basic_blocks().iter().for_each(
        |bb| {
            debug!("Node: {:?}", bb);
        }
    );
    
    debug!("Start Node: {:?}", _to_print_input_body.start_node());
    _to_print_input_body.successors(_to_print_input_body.start_node()).for_each(
        |bb| {
            debug!("Successor to Start: {:?}", bb);
        }
    );
    // let _to_print_promoted: &IndexVec<_, _> = &_promoted.borrow();

    // println!("{:?}", _to_print_input_body);
    // println!("{:?}", _to_print_promoted);

    // println!("run query mir_symbolic_exec: {}", tcx.def_path_str(def.did.to_def_id()));
    // let hir_owner = tcx.hir().local_def_id_to_hir_id(def.did).owner;
    //
    // let opt_closure_req = tcx.infer_ctxt().with_opaque_type_inference(hir_owner).enter(|_infcx| {
    //     let input_body: &Body<'_> = &input_body.borrow();
    //     let promoted: &IndexVec<_, _> = &promoted.borrow();
    //     println!("hi from mit_symbolic_exec closure");
    //     let result = BorrowCheckResult {
    //         concrete_opaque_types: VecMap::new(),
    //         closure_requirements: None,
    //         used_mut_upvars: SmallVec::new(),
    //         tainted_by_errors: None
    //     };
    //     println!("{:?}", input_body);
    //     println!("{:?}", promoted);
    //     result
    //     // pub concrete_opaque_types: VecMap<DefId, OpaqueHiddenType<'tcx>>,
    //     // pub closure_requirements: Option<ClosureRegionRequirements<'tcx>>,
    //     // pub used_mut_upvars: SmallVec<[Field; 8]>,
    //     // pub tainted_by_errors: Option<ErrorGuaranteed>,
    // });

    let result = BorrowCheckResult {
        concrete_opaque_types: VecMap::new(),
        closure_requirements: None,
        used_mut_upvars: SmallVec::new(),
        tainted_by_errors: None,
    };

    debug!("mir_symbolic_exec done");

    tcx.arena.alloc(result)
}
