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
use u_get_VTTBR_Type_VMID::*;
use HaveCommonNotPrivateTransExt::*;
use u_get_VTTBR_Type_CnP::*;
use common::*;
pub fn AArch32_GetS2TLBContext<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ipa: ProductTypeda0231e9dc169f81,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        ipa: ProductTypeda0231e9dc169f81,
    }
    let fn_state = FunctionState {
        ipa,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_0_0: read-var ipa.1:struct
        let s_0_0: u32 = fn_state.ipa._1;
        // C s_0_1: const #0u : u32
        let s_0_1: u32 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: assert s_0_2
        let s_0_3: () = assert!(s_0_2);
        // C s_0_4: const #0u : u32
        let s_0_4: u32 = 0;
        // D s_0_5: write-var tlbcontext.11 <= s_0_4
        fn_state.tlbcontext._11 = s_0_4;
        // C s_0_6: const #4u : u32
        let s_0_6: u32 = 4;
        // D s_0_7: write-var tlbcontext.10 <= s_0_6
        fn_state.tlbcontext._10 = s_0_6;
        // D s_0_8: read-var ipa.1:struct
        let s_0_8: u32 = fn_state.ipa._1;
        // D s_0_9: write-var tlbcontext.6 <= s_0_8
        fn_state.tlbcontext._6 = s_0_8;
        // C s_0_10: const #22408u : u32
        let s_0_10: u32 = 22408;
        // D s_0_11: read-reg s_0_10:struct
        let s_0_11: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_10 as isize);
            tracer.read_register(s_0_10 as isize, value);
            value
        };
        // D s_0_12: call _get_VTTBR_Type_VMID(s_0_11)
        let s_0_12: u8 = u_get_VTTBR_Type_VMID(state, tracer, s_0_11);
        // C s_0_13: const #16s : i
        let s_0_13: i128 = 16;
        // D s_0_14: cast zx s_0_12 -> bv
        let s_0_14: Bits = Bits::new(s_0_12 as u128, 8u16);
        // D s_0_15: bits-cast zx s_0_14 -> bv length s_0_13
        let s_0_15: Bits = s_0_14.zero_extend(s_0_13);
        // D s_0_16: cast reint s_0_15 -> u16
        let s_0_16: u16 = (s_0_15.value() as u16);
        // D s_0_17: write-var tlbcontext.13 <= s_0_16
        fn_state.tlbcontext._13 = s_0_16;
        // C s_0_18: const #0u : u32
        let s_0_18: u32 = 0;
        // D s_0_19: write-var tlbcontext.12 <= s_0_18
        fn_state.tlbcontext._12 = s_0_18;
        // C s_0_20: const #0u : u8
        let s_0_20: bool = false;
        // D s_0_21: write-var tlbcontext.4 <= s_0_20
        fn_state.tlbcontext._4 = s_0_20;
        // C s_0_22: const #1u : u8
        let s_0_22: bool = true;
        // D s_0_23: write-var tlbcontext.5 <= s_0_22
        fn_state.tlbcontext._5 = s_0_22;
        // D s_0_24: read-var ipa.0:struct
        let s_0_24: u64 = fn_state.ipa._0;
        // C s_0_25: const #64s : i
        let s_0_25: i128 = 64;
        // D s_0_26: cast zx s_0_24 -> bv
        let s_0_26: Bits = Bits::new(s_0_24 as u128, 56u16);
        // D s_0_27: bits-cast zx s_0_26 -> bv length s_0_25
        let s_0_27: Bits = s_0_26.zero_extend(s_0_25);
        // D s_0_28: cast reint s_0_27 -> u64
        let s_0_28: u64 = (s_0_27.value() as u64);
        // D s_0_29: write-var tlbcontext.2 <= s_0_28
        fn_state.tlbcontext._2 = s_0_28;
        // C s_0_30: const #() : ()
        let s_0_30: () = ();
        // S s_0_31: call HaveCommonNotPrivateTransExt(s_0_30)
        let s_0_31: bool = HaveCommonNotPrivateTransExt(state, tracer, s_0_30);
        // N s_0_32: branch s_0_31 b3 b1
        if s_0_31 {
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
        // D s_2_0: read-var tlbcontext:struct
        let s_2_0: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_3_0: const #22408u : u32
        let s_3_0: u32 = 22408;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_VTTBR_Type_CnP(s_3_1)
        let s_3_2: bool = u_get_VTTBR_Type_CnP(state, tracer, s_3_1);
        // D s_3_3: write-var tlbcontext.1 <= s_3_2
        fn_state.tlbcontext._1 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
