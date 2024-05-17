#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use VTTBR_EL2_read::*;
use VMID_read::*;
use EL2Enabled::*;
use HaveCommonNotPrivateTransExt::*;
use u_get_VTTBR_EL2_Type_CnP::*;
use u_get_VSTTBR_EL2_Type_CnP::*;
use common::*;
pub fn AArch64_GetS2TLBContext<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    ipa: ProductTypeda0231e9dc169f81,
    tg: u32,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        ss: u32,
        ipa: ProductTypeda0231e9dc169f81,
        tg: u32,
    }
    let fn_state = FunctionState {
        ss,
        ipa,
        tg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EL2Enabled(s_0_0)
        let s_0_1: bool = EL2Enabled(state, tracer, s_0_0);
        // N s_0_2: assert s_0_1
        let s_0_2: () = assert!(s_0_1);
        // D s_0_3: read-var ss:u32
        let s_0_3: u32 = fn_state.ss;
        // D s_0_4: write-var tlbcontext.11 <= s_0_3
        fn_state.tlbcontext._11 = s_0_3;
        // C s_0_5: const #4u : u32
        let s_0_5: u32 = 4;
        // D s_0_6: write-var tlbcontext.10 <= s_0_5
        fn_state.tlbcontext._10 = s_0_5;
        // D s_0_7: read-var ipa.1:struct
        let s_0_7: u32 = fn_state.ipa._1;
        // D s_0_8: write-var tlbcontext.6 <= s_0_7
        fn_state.tlbcontext._6 = s_0_7;
        // C s_0_9: const #() : ()
        let s_0_9: () = ();
        // S s_0_10: call VMID_read(s_0_9)
        let s_0_10: u16 = VMID_read(state, tracer, s_0_9);
        // D s_0_11: write-var tlbcontext.13 <= s_0_10
        fn_state.tlbcontext._13 = s_0_10;
        // D s_0_12: read-var tg:u32
        let s_0_12: u32 = fn_state.tg;
        // D s_0_13: write-var tlbcontext.12 <= s_0_12
        fn_state.tlbcontext._12 = s_0_12;
        // D s_0_14: read-var ipa.0:struct
        let s_0_14: u64 = fn_state.ipa._0;
        // C s_0_15: const #64s : i
        let s_0_15: i128 = 64;
        // D s_0_16: cast zx s_0_14 -> bv
        let s_0_16: Bits = Bits::new(s_0_14 as u128, 56u16);
        // D s_0_17: bits-cast zx s_0_16 -> bv length s_0_15
        let s_0_17: Bits = s_0_16.zero_extend(s_0_15);
        // D s_0_18: cast reint s_0_17 -> u64
        let s_0_18: u64 = (s_0_17.value() as u64);
        // D s_0_19: write-var tlbcontext.2 <= s_0_18
        fn_state.tlbcontext._2 = s_0_18;
        // C s_0_20: const #() : ()
        let s_0_20: () = ();
        // S s_0_21: call HaveCommonNotPrivateTransExt(s_0_20)
        let s_0_21: bool = HaveCommonNotPrivateTransExt(state, tracer, s_0_20);
        // N s_0_22: branch s_0_21 b3 b1
        if s_0_21 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var tlbcontext.1 <= s_1_0
        fn_state.tlbcontext._1 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var tlbcontext.4 <= s_2_0
        fn_state.tlbcontext._4 = s_2_0;
        // C s_2_2: const #1u : u8
        let s_2_2: bool = true;
        // D s_2_3: write-var tlbcontext.5 <= s_2_2
        fn_state.tlbcontext._5 = s_2_2;
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // D s_2_5: write-var tlbcontext.3 <= s_2_4
        fn_state.tlbcontext._3 = s_2_4;
        // D s_2_6: read-var tlbcontext:struct
        let s_2_6: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_2_7: return s_2_6
        return s_2_6;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_3_0: read-var ipa.1:struct
        let s_3_0: u32 = fn_state.ipa._1;
        // C s_3_1: const #1u : u32
        let s_3_1: u32 = 1;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b6 b4
        if s_3_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call VTTBR_EL2_read(s_4_0)
        let s_4_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_4_0);
        // S s_4_2: call _get_VTTBR_EL2_Type_CnP(s_4_1)
        let s_4_2: bool = u_get_VTTBR_EL2_Type_CnP(state, tracer, s_4_1);
        // D s_4_3: write-var tlbcontext.1 <= s_4_2
        fn_state.tlbcontext._1 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // N s_5_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_6_0: const #22824u : u32
        let s_6_0: u32 = 22824;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_VSTTBR_EL2_Type_CnP(s_6_1)
        let s_6_2: bool = u_get_VSTTBR_EL2_Type_CnP(state, tracer, s_6_1);
        // D s_6_3: write-var tlbcontext.1 <= s_6_2
        fn_state.tlbcontext._1 = s_6_2;
        // N s_6_4: jump b5
        return block_5(state, tracer, fn_state);
    }
}
