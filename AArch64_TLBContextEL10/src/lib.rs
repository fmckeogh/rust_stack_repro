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
use Zeros::*;
use u_get_TCR_EL1_Type_A1::*;
use u_get_TTBR1_EL1_Type_CnP::*;
use u_get_TTBR0_EL1_Type_CnP::*;
use u_get_TCR_EL1_Type_AS::*;
use u_get_TTBR1_EL1_Type_ASID::*;
use u_get_TTBR0_EL1_Type_ASID::*;
use AArch64_GetVARange::*;
use TTBR1_EL1_read::*;
use VMID_read::*;
use HaveCommonNotPrivateTransExt::*;
use TTBR0_EL1_read::*;
use common::*;
pub fn AArch64_TLBContextEL10<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    va: u64,
    tg: u32,
) -> ProductTypec0d0fb0603850c4c {
    #[derive(Default)]
    struct FunctionState {
        tlbcontext: ProductTypec0d0fb0603850c4c,
        ss: u32,
        va: u64,
        tg: u32,
    }
    let fn_state = FunctionState {
        ss,
        va,
        tg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_0_0: read-var ss:u32
        let s_0_0: u32 = fn_state.ss;
        // D s_0_1: write-var tlbcontext.11 <= s_0_0
        fn_state.tlbcontext._11 = s_0_0;
        // C s_0_2: const #4u : u32
        let s_0_2: u32 = 4;
        // D s_0_3: write-var tlbcontext.10 <= s_0_2
        fn_state.tlbcontext._10 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call VMID_read(s_0_4)
        let s_0_5: u16 = VMID_read(state, tracer, s_0_4);
        // D s_0_6: write-var tlbcontext.13 <= s_0_5
        fn_state.tlbcontext._13 = s_0_5;
        // C s_0_7: const #22392u : u32
        let s_0_7: u32 = 22392;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_TCR_EL1_Type_A1(s_0_8)
        let s_0_9: bool = u_get_TCR_EL1_Type_A1(state, tracer, s_0_8);
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // C s_0_11: const #0u : u8
        let s_0_11: bool = false;
        // C s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 1u16);
        // D s_0_13: cmp-eq s_0_10 s_0_12
        let s_0_13: bool = ((s_0_10) == (s_0_12));
        // N s_0_14: branch s_0_13 b11 b1
        if s_0_13 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call TTBR1_EL1_read(s_1_0)
        let s_1_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_1_0);
        // S s_1_2: call _get_TTBR1_EL1_Type_ASID(s_1_1)
        let s_1_2: u16 = u_get_TTBR1_EL1_Type_ASID(state, tracer, s_1_1);
        // D s_1_3: write-var tlbcontext.0 <= s_1_2
        fn_state.tlbcontext._0 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_2_0: const #22392u : u32
        let s_2_0: u32 = 22392;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_TCR_EL1_Type_AS(s_2_1)
        let s_2_2: bool = u_get_TCR_EL1_Type_AS(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b10 b3
        if s_2_6 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_4_0: read-var tg:u32
        let s_4_0: u32 = fn_state.tg;
        // D s_4_1: write-var tlbcontext.12 <= s_4_0
        fn_state.tlbcontext._12 = s_4_0;
        // D s_4_2: read-var va:u64
        let s_4_2: u64 = fn_state.va;
        // D s_4_3: write-var tlbcontext.2 <= s_4_2
        fn_state.tlbcontext._2 = s_4_2;
        // C s_4_4: const #() : ()
        let s_4_4: () = ();
        // S s_4_5: call HaveCommonNotPrivateTransExt(s_4_4)
        let s_4_5: bool = HaveCommonNotPrivateTransExt(state, tracer, s_4_4);
        // N s_4_6: branch s_4_5 b7 b5
        if s_4_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var tlbcontext.1 <= s_5_0
        fn_state.tlbcontext._1 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_6_0: read-var tlbcontext:struct
        let s_6_0: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // D s_7_0: read-var va:u64
        let s_7_0: u64 = fn_state.va;
        // D s_7_1: call AArch64_GetVARange(s_7_0)
        let s_7_1: u32 = AArch64_GetVARange(state, tracer, s_7_0);
        // C s_7_2: const #0u : u32
        let s_7_2: u32 = 0;
        // D s_7_3: cmp-eq s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) == (s_7_2));
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call TTBR1_EL1_read(s_8_0)
        let s_8_1: ProductType782ac6922b48c20d = TTBR1_EL1_read(state, tracer, s_8_0);
        // S s_8_2: call _get_TTBR1_EL1_Type_CnP(s_8_1)
        let s_8_2: bool = u_get_TTBR1_EL1_Type_CnP(state, tracer, s_8_1);
        // D s_8_3: write-var tlbcontext.1 <= s_8_2
        fn_state.tlbcontext._1 = s_8_2;
        // N s_8_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call TTBR0_EL1_read(s_9_0)
        let s_9_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_9_0);
        // S s_9_2: call _get_TTBR0_EL1_Type_CnP(s_9_1)
        let s_9_2: bool = u_get_TTBR0_EL1_Type_CnP(state, tracer, s_9_1);
        // D s_9_3: write-var tlbcontext.1 <= s_9_2
        fn_state.tlbcontext._1 = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_10_0: const #8s : i
        let s_10_0: i128 = 8;
        // S s_10_1: call Zeros(s_10_0)
        let s_10_1: Bits = Zeros(state, tracer, s_10_0);
        // D s_10_2: read-var tlbcontext:struct
        let s_10_2: ProductTypec0d0fb0603850c4c = fn_state.tlbcontext;
        // D s_10_3: write-var tlbcontext <= s_10_2
        fn_state.tlbcontext = s_10_2;
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec0d0fb0603850c4c {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call TTBR0_EL1_read(s_11_0)
        let s_11_1: ProductType782ac6922b48c20d = TTBR0_EL1_read(state, tracer, s_11_0);
        // S s_11_2: call _get_TTBR0_EL1_Type_ASID(s_11_1)
        let s_11_2: u16 = u_get_TTBR0_EL1_Type_ASID(state, tracer, s_11_1);
        // D s_11_3: write-var tlbcontext.0 <= s_11_2
        fn_state.tlbcontext._0 = s_11_2;
        // N s_11_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
